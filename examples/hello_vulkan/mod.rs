//! This example is based on [ash-examples][1] code.
//!
//! [1]:
//! https://github.com/ash-rs/ash/tree/master/ash-examples

mod base;
mod helpers;

use std::{ffi, io::Cursor, mem, mem::offset_of};

use ash::{util::*, vk};
use b3_core::{
    Action,
    ActiveApplication,
    Application,
    ContextOwner,
    Event,
    EventHandler,
    LifeCycle,
    Menu,
    MenuItem,
    Size,
    Window,
};
use helpers::record_submit_commandbuffer;

use crate::{base::Base, helpers::find_memorytype_index};

#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos:   [f32; 4],
    color: [f32; 4],
}

fn create_menu(ctx: &impl ContextOwner) -> Menu {
    // App menu
    let quit_item = MenuItem::builder()
        .with_title("Quit")
        .with_macos_short_code("q")
        .with_action(Action::new_event("quit"))
        .build(ctx);

    let app_menu = Menu::builder().with_item(quit_item).build(ctx);

    // Main menu
    let app_item = MenuItem::builder().with_submenu(app_menu).build(ctx);

    Menu::builder().with_item(app_item).build(ctx)
}

struct State {
    menu:   Menu,
    window: Window,
    base:   Base,
}

impl State {
    unsafe fn new(ctx: &impl ContextOwner) -> Self {
        let menu = create_menu(ctx);
        let size = Size::new(1024, 800);
        let window = Window::builder()
            .with_title("Hello, Vulkan!")
            .with_size(size.clone())
            .build(ctx);

        let base = Base::new(&window);

        Self {
            menu,
            window,
            base,
        }
    }
}

impl EventHandler for State {
    fn on_event(&mut self, app: &mut ActiveApplication, event: Event) {
        match event {
            Event::Menu(action) => {
                if action == "quit" {
                    app.stop()
                }
            }
            Event::LifeCycle(LifeCycle::Start) => {
                app.set_menu(Some(&self.menu));
                self.window.show(app);

                unsafe {
                    let renderpass_attachments = [
                        vk::AttachmentDescription {
                            format: self.base.surface_format.format,
                            samples: vk::SampleCountFlags::TYPE_1,
                            load_op: vk::AttachmentLoadOp::CLEAR,
                            store_op: vk::AttachmentStoreOp::STORE,
                            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                            ..Default::default()
                        },
                        vk::AttachmentDescription {
                            format: vk::Format::D16_UNORM,
                            samples: vk::SampleCountFlags::TYPE_1,
                            load_op: vk::AttachmentLoadOp::CLEAR,
                            initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                            ..Default::default()
                        },
                    ];
                    let color_attachment_refs = [vk::AttachmentReference {
                        attachment: 0,
                        layout:     vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                    }];
                    let depth_attachment_ref = vk::AttachmentReference {
                        attachment: 1,
                        layout:     vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    };
                    let dependencies = [vk::SubpassDependency {
                        src_subpass: vk::SUBPASS_EXTERNAL,
                        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                            | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                        ..Default::default()
                    }];

                    let subpass = vk::SubpassDescription::default()
                        .color_attachments(&color_attachment_refs)
                        .depth_stencil_attachment(&depth_attachment_ref)
                        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

                    let renderpass_create_info = vk::RenderPassCreateInfo::default()
                        .attachments(&renderpass_attachments)
                        .subpasses(std::slice::from_ref(&subpass))
                        .dependencies(&dependencies);

                    let renderpass = self
                        .base
                        .device
                        .create_render_pass(&renderpass_create_info, None)
                        .unwrap();

                    let framebuffers: Vec<vk::Framebuffer> = self
                        .base
                        .present_image_views
                        .iter()
                        .map(|&present_image_view| {
                            let framebuffer_attachments =
                                [present_image_view, self.base.depth_image_view];
                            let frame_buffer_create_info = vk::FramebufferCreateInfo::default()
                                .render_pass(renderpass)
                                .attachments(&framebuffer_attachments)
                                .width(self.base.surface_resolution.width)
                                .height(self.base.surface_resolution.height)
                                .layers(1);

                            self.base
                                .device
                                .create_framebuffer(&frame_buffer_create_info, None)
                                .unwrap()
                        })
                        .collect();

                    let index_buffer_data = [0u32, 1, 2];
                    let index_buffer_info = vk::BufferCreateInfo::default()
                        .size(mem::size_of_val(&index_buffer_data) as u64)
                        .usage(vk::BufferUsageFlags::INDEX_BUFFER)
                        .sharing_mode(vk::SharingMode::EXCLUSIVE);

                    let index_buffer = self
                        .base
                        .device
                        .create_buffer(&index_buffer_info, None)
                        .unwrap();
                    let index_buffer_memory_req = self
                        .base
                        .device
                        .get_buffer_memory_requirements(index_buffer);
                    let index_buffer_memory_index = find_memorytype_index(
                        &index_buffer_memory_req,
                        &self.base.device_memory_properties,
                        vk::MemoryPropertyFlags::HOST_VISIBLE
                            | vk::MemoryPropertyFlags::HOST_COHERENT,
                    )
                    .expect("Unable to find suitable memorytype for the index buffer.");

                    let index_allocate_info = vk::MemoryAllocateInfo {
                        allocation_size: index_buffer_memory_req.size,
                        memory_type_index: index_buffer_memory_index,
                        ..Default::default()
                    };
                    let index_buffer_memory = self
                        .base
                        .device
                        .allocate_memory(&index_allocate_info, None)
                        .unwrap();
                    let index_ptr = self
                        .base
                        .device
                        .map_memory(
                            index_buffer_memory,
                            0,
                            index_buffer_memory_req.size,
                            vk::MemoryMapFlags::empty(),
                        )
                        .unwrap();
                    let mut index_slice = Align::new(
                        index_ptr,
                        mem::align_of::<u32>() as u64,
                        index_buffer_memory_req.size,
                    );
                    index_slice.copy_from_slice(&index_buffer_data);
                    self.base.device.unmap_memory(index_buffer_memory);
                    self.base
                        .device
                        .bind_buffer_memory(index_buffer, index_buffer_memory, 0)
                        .unwrap();

                    let vertex_input_buffer_info = vk::BufferCreateInfo {
                        size: 3 * mem::size_of::<Vertex>() as u64,
                        usage: vk::BufferUsageFlags::VERTEX_BUFFER,
                        sharing_mode: vk::SharingMode::EXCLUSIVE,
                        ..Default::default()
                    };

                    let vertex_input_buffer = self
                        .base
                        .device
                        .create_buffer(&vertex_input_buffer_info, None)
                        .unwrap();

                    let vertex_input_buffer_memory_req = self
                        .base
                        .device
                        .get_buffer_memory_requirements(vertex_input_buffer);

                    let vertex_input_buffer_memory_index = find_memorytype_index(
                        &vertex_input_buffer_memory_req,
                        &self.base.device_memory_properties,
                        vk::MemoryPropertyFlags::HOST_VISIBLE
                            | vk::MemoryPropertyFlags::HOST_COHERENT,
                    )
                    .expect("Unable to find suitable memorytype for the vertex buffer.");

                    let vertex_buffer_allocate_info = vk::MemoryAllocateInfo {
                        allocation_size: vertex_input_buffer_memory_req.size,
                        memory_type_index: vertex_input_buffer_memory_index,
                        ..Default::default()
                    };

                    let vertex_input_buffer_memory = self
                        .base
                        .device
                        .allocate_memory(&vertex_buffer_allocate_info, None)
                        .unwrap();

                    let vertices = [
                        Vertex {
                            pos:   [-1.0, 1.0, 0.0, 1.0],
                            color: [0.0, 1.0, 0.0, 1.0],
                        },
                        Vertex {
                            pos:   [1.0, 1.0, 0.0, 1.0],
                            color: [0.0, 0.0, 1.0, 1.0],
                        },
                        Vertex {
                            pos:   [0.0, -1.0, 0.0, 1.0],
                            color: [1.0, 0.0, 0.0, 1.0],
                        },
                    ];

                    let vert_ptr = self
                        .base
                        .device
                        .map_memory(
                            vertex_input_buffer_memory,
                            0,
                            vertex_input_buffer_memory_req.size,
                            vk::MemoryMapFlags::empty(),
                        )
                        .unwrap();

                    let mut vert_align = Align::new(
                        vert_ptr,
                        mem::align_of::<Vertex>() as u64,
                        vertex_input_buffer_memory_req.size,
                    );
                    vert_align.copy_from_slice(&vertices);
                    self.base.device.unmap_memory(vertex_input_buffer_memory);
                    self.base
                        .device
                        .bind_buffer_memory(vertex_input_buffer, vertex_input_buffer_memory, 0)
                        .unwrap();

                    let mut vertex_spv_file = Cursor::new(&include_bytes!("shaders/vert.spv")[..]);
                    let mut frag_spv_file = Cursor::new(&include_bytes!("shaders/frag.spv")[..]);

                    let vertex_code = read_spv(&mut vertex_spv_file)
                        .expect("Failed to read vertex shader spv file");
                    let vertex_shader_info =
                        vk::ShaderModuleCreateInfo::default().code(&vertex_code);

                    let frag_code = read_spv(&mut frag_spv_file)
                        .expect("Failed to read fragment shader spv file");
                    let frag_shader_info = vk::ShaderModuleCreateInfo::default().code(&frag_code);

                    let vertex_shader_module = self
                        .base
                        .device
                        .create_shader_module(&vertex_shader_info, None)
                        .expect("Vertex shader module error");

                    let fragment_shader_module = self
                        .base
                        .device
                        .create_shader_module(&frag_shader_info, None)
                        .expect("Fragment shader module error");

                    let layout_create_info = vk::PipelineLayoutCreateInfo::default();

                    let pipeline_layout = self
                        .base
                        .device
                        .create_pipeline_layout(&layout_create_info, None)
                        .unwrap();

                    let shader_entry_name = ffi::CStr::from_bytes_with_nul_unchecked(b"main\0");
                    let shader_stage_create_infos = [
                        vk::PipelineShaderStageCreateInfo {
                            module: vertex_shader_module,
                            p_name: shader_entry_name.as_ptr(),
                            stage: vk::ShaderStageFlags::VERTEX,
                            ..Default::default()
                        },
                        vk::PipelineShaderStageCreateInfo {
                            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                            module: fragment_shader_module,
                            p_name: shader_entry_name.as_ptr(),
                            stage: vk::ShaderStageFlags::FRAGMENT,
                            ..Default::default()
                        },
                    ];
                    let vertex_input_binding_descriptions = [vk::VertexInputBindingDescription {
                        binding:    0,
                        stride:     mem::size_of::<Vertex>() as u32,
                        input_rate: vk::VertexInputRate::VERTEX,
                    }];
                    let vertex_input_attribute_descriptions = [
                        vk::VertexInputAttributeDescription {
                            location: 0,
                            binding:  0,
                            format:   vk::Format::R32G32B32A32_SFLOAT,
                            offset:   offset_of!(Vertex, pos) as u32,
                        },
                        vk::VertexInputAttributeDescription {
                            location: 1,
                            binding:  0,
                            format:   vk::Format::R32G32B32A32_SFLOAT,
                            offset:   offset_of!(Vertex, color) as u32,
                        },
                    ];

                    let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
                        .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
                        .vertex_binding_descriptions(&vertex_input_binding_descriptions);
                    let vertex_input_assembly_state_info =
                        vk::PipelineInputAssemblyStateCreateInfo {
                            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                            ..Default::default()
                        };

                    let viewports = [vk::Viewport {
                        x:         0.0,
                        y:         0.0,
                        width:     self.base.surface_resolution.width as f32,
                        height:    self.base.surface_resolution.height as f32,
                        min_depth: 0.0,
                        max_depth: 1.0,
                    }];
                    let scissors = [self.base.surface_resolution.into()];
                    let viewport_state_info = vk::PipelineViewportStateCreateInfo::default()
                        .scissors(&scissors)
                        .viewports(&viewports);

                    let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
                        front_face: vk::FrontFace::COUNTER_CLOCKWISE,
                        line_width: 1.0,
                        polygon_mode: vk::PolygonMode::FILL,
                        ..Default::default()
                    };
                    let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
                        rasterization_samples: vk::SampleCountFlags::TYPE_1,
                        ..Default::default()
                    };
                    let noop_stencil_state = vk::StencilOpState {
                        fail_op: vk::StencilOp::KEEP,
                        pass_op: vk::StencilOp::KEEP,
                        depth_fail_op: vk::StencilOp::KEEP,
                        compare_op: vk::CompareOp::ALWAYS,
                        ..Default::default()
                    };
                    let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
                        depth_test_enable: 1,
                        depth_write_enable: 1,
                        depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
                        front: noop_stencil_state,
                        back: noop_stencil_state,
                        max_depth_bounds: 1.0,
                        ..Default::default()
                    };
                    let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
                        blend_enable:           0,
                        src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
                        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
                        color_blend_op:         vk::BlendOp::ADD,
                        src_alpha_blend_factor: vk::BlendFactor::ZERO,
                        dst_alpha_blend_factor: vk::BlendFactor::ZERO,
                        alpha_blend_op:         vk::BlendOp::ADD,
                        color_write_mask:       vk::ColorComponentFlags::RGBA,
                    }];
                    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
                        .logic_op(vk::LogicOp::CLEAR)
                        .attachments(&color_blend_attachment_states);

                    let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
                    let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
                        .dynamic_states(&dynamic_state);

                    let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::default()
                        .stages(&shader_stage_create_infos)
                        .vertex_input_state(&vertex_input_state_info)
                        .input_assembly_state(&vertex_input_assembly_state_info)
                        .viewport_state(&viewport_state_info)
                        .rasterization_state(&rasterization_info)
                        .multisample_state(&multisample_state_info)
                        .depth_stencil_state(&depth_state_info)
                        .color_blend_state(&color_blend_state)
                        .dynamic_state(&dynamic_state_info)
                        .layout(pipeline_layout)
                        .render_pass(renderpass);

                    let graphics_pipelines = self
                        .base
                        .device
                        .create_graphics_pipelines(
                            vk::PipelineCache::null(),
                            &[graphic_pipeline_info],
                            None,
                        )
                        .expect("Unable to create graphics pipeline");

                    let graphic_pipeline = graphics_pipelines[0];

                    let (present_index, _) = self
                        .base
                        .swapchain_loader
                        .acquire_next_image(
                            self.base.swapchain,
                            std::u64::MAX,
                            self.base.present_complete_semaphore,
                            vk::Fence::null(),
                        )
                        .unwrap();
                    let clear_values = [
                        vk::ClearValue {
                            color: vk::ClearColorValue {
                                float32: [0.0, 0.0, 0.0, 0.0],
                            },
                        },
                        vk::ClearValue {
                            depth_stencil: vk::ClearDepthStencilValue {
                                depth:   1.0,
                                stencil: 0,
                            },
                        },
                    ];

                    let render_pass_begin_info = vk::RenderPassBeginInfo::default()
                        .render_pass(renderpass)
                        .framebuffer(framebuffers[present_index as usize])
                        .render_area(self.base.surface_resolution.into())
                        .clear_values(&clear_values);

                    record_submit_commandbuffer(
                        &self.base.device,
                        self.base.draw_command_buffer,
                        self.base.draw_commands_reuse_fence,
                        self.base.present_queue,
                        &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
                        &[self.base.present_complete_semaphore],
                        &[self.base.rendering_complete_semaphore],
                        |device, draw_command_buffer| {
                            device.cmd_begin_render_pass(
                                draw_command_buffer,
                                &render_pass_begin_info,
                                vk::SubpassContents::INLINE,
                            );
                            device.cmd_bind_pipeline(
                                draw_command_buffer,
                                vk::PipelineBindPoint::GRAPHICS,
                                graphic_pipeline,
                            );
                            device.cmd_set_viewport(draw_command_buffer, 0, &viewports);
                            device.cmd_set_scissor(draw_command_buffer, 0, &scissors);
                            device.cmd_bind_vertex_buffers(
                                draw_command_buffer,
                                0,
                                &[vertex_input_buffer],
                                &[0],
                            );
                            device.cmd_bind_index_buffer(
                                draw_command_buffer,
                                index_buffer,
                                0,
                                vk::IndexType::UINT32,
                            );
                            device.cmd_draw_indexed(
                                draw_command_buffer,
                                index_buffer_data.len() as u32,
                                1,
                                0,
                                0,
                                1,
                            );
                            // Or draw without the index buffer
                            // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                            device.cmd_end_render_pass(draw_command_buffer);
                        },
                    );
                    let wait_semaphors = [self.base.rendering_complete_semaphore];
                    let swapchains = [self.base.swapchain];
                    let image_indices = [present_index];
                    let present_info = vk::PresentInfoKHR::default()
                        .wait_semaphores(&wait_semaphors) // &base.rendering_complete_semaphore)
                        .swapchains(&swapchains)
                        .image_indices(&image_indices);

                    self.base
                        .swapchain_loader
                        .queue_present(self.base.present_queue, &present_info)
                        .unwrap();

                    self.base.device.device_wait_idle().unwrap();
                    for pipeline in graphics_pipelines {
                        self.base.device.destroy_pipeline(pipeline, None);
                    }
                    self.base
                        .device
                        .destroy_pipeline_layout(pipeline_layout, None);
                    self.base
                        .device
                        .destroy_shader_module(vertex_shader_module, None);
                    self.base
                        .device
                        .destroy_shader_module(fragment_shader_module, None);
                    self.base.device.free_memory(index_buffer_memory, None);
                    self.base.device.destroy_buffer(index_buffer, None);
                    self.base
                        .device
                        .free_memory(vertex_input_buffer_memory, None);
                    self.base.device.destroy_buffer(vertex_input_buffer, None);
                    self.base.device.device_wait_idle().unwrap();

                    for framebuffer in framebuffers {
                        self.base.device.destroy_framebuffer(framebuffer, None);
                    }
                    self.base.device.destroy_render_pass(renderpass, None);
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let app = Application::new().unwrap();
    let state = unsafe { State::new(&app) };
    app.run(state);
}
