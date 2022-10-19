#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                    // format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct BarBrush {
    pub buffers: (wgpu::Buffer, wgpu::Buffer),
    pub shader: wgpu::ShaderModule,
    pub vertices: Vec<Vertex>,
    pub num_indices: u32,
}

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4];

impl BarBrush {
    pub fn new(
        device: &dyn wgpu::util::DeviceExt,
        shader: wgpu::ShaderModule,
    ) -> BarBrush {
        let vertices: &[Vertex] = &[
            Vertex {
                position: [-2.0, 1.5],
                color: [0.94, 0.47, 0.0],
            },
            Vertex {
                position: [-2.0, 0.83],
                color: [0.5, 0.0, 0.5],
            },
            Vertex {
                position: [2.0, 0.83],
                color: [0.94, 0.47, 0.0],
            },
            Vertex {
                position: [-2.0, 2.0],
                color: [0.827, 0.317, 0.0],
            },
            Vertex {
                position: [-2.0, 0.87],
                color: [0.5, 0.0, 0.5],
            },
            Vertex {
                position: [2.0, 0.87],
                color: [0.827, 0.317, 0.0],
            },
        ];

        let vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Bar::Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Bar::Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices: u32 = INDICES.len() as u32;

        BarBrush {
            buffers: (vertex_buffer, index_buffer),
            vertices: vertices.to_vec(),
            shader,
            num_indices,
        }
    }
}