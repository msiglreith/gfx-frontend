
extern crate gfx;
extern crate gfx_core as core;
extern crate gfx_device_gl;
extern crate gfx_device_dx11;

use core::{buffer, format, handle, mapping, pso, texture};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Resources { }

impl core::Resources for Resources {
    type Buffer              = Buffer;
    type Shader              = Shader;
    type Program             = Program;
    type PipelineStateObject = PipelineState;
    type Texture             = Texture;
    type RenderTargetView    = Rtv;
    type DepthStencilView    = Dsv;
    type ShaderResourceView  = Srv;
    type UnorderedAccessView = Uav;
    type Sampler             = Sampler;
    type Fence               = Fence;
    type Mapping             = Mapping;
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Buffer {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::Buffer),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::Buffer),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Shader {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::Shader),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::Shader),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Program {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::Program),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::Program),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PipelineState {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::PipelineStateObject),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::PipelineStateObject),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Texture {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::Texture),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::Texture),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Rtv {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::RenderTargetView),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::RenderTargetView),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dsv {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::DepthStencilView),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::DepthStencilView),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Srv {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::ShaderResourceView),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::ShaderResourceView),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Uav {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::UnorderedAccessView),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::UnorderedAccessView),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Sampler {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::Sampler),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::Sampler),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Fence {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::Fence),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::Fence),
}

impl core::Fence for Fence {
    fn wait(&self) {
        match *self {
            Fence::OpenGL(ref fence) => fence.wait(),
            Fence::D3D11(ref fence) => fence.wait(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Mapping {
    OpenGL(<gfx_device_gl::Resources as core::Resources>::Mapping),
    D3D11(<gfx_device_dx11::Resources as core::Resources>::Mapping),
}

impl core::mapping::Gate<Resources> for Mapping {
    unsafe fn set<T>(&self, index: usize, val: T) {
        match *self {
            Mapping::OpenGL(ref mapping) => mapping.set(index, val),
            Mapping::D3D11(ref mapping) => mapping.set(index, val),
        }
    }

    unsafe fn slice<'a, 'b, T>(&'a self, len: usize) -> &'b [T] {
        match *self {
            Mapping::OpenGL(ref mapping) => mapping.slice(len),
            Mapping::D3D11(ref mapping) => mapping.slice(len),
        }
    }

    unsafe fn mut_slice<'a, 'b, T>(&'a self, len: usize) -> &'b mut [T] {
        match *self {
            Mapping::OpenGL(ref mapping) => mapping.mut_slice(len),
            Mapping::D3D11(ref mapping) => mapping.mut_slice(len),
        }
    }
}

pub enum Factory {
    OpenGL(gfx_device_gl::Factory),
    D3D11(gfx_device_dx11::Factory),
}

impl core::Factory<Resources> for Factory {
    fn get_capabilities(&self) -> &core::Capabilities {
        match *self {
            Factory::OpenGL(ref factory) => factory.get_capabilities(),
            Factory::D3D11(ref factory) => factory.get_capabilities(),
        }
    }

    fn create_buffer_raw(&mut self, info: buffer::Info) -> Result<handle::RawBuffer<Resources>, buffer::CreationError> {
        unimplemented!()
    }
    
    fn create_buffer_immutable_raw(&mut self, data: &[u8], stride: usize, role: buffer::Role, bind: core::memory::Bind)
                               -> Result<handle::RawBuffer<Resources>, buffer::CreationError> {
        unimplemented!()
    }

    fn create_shader(&mut self, _stage: core::shade::Stage, _code: &[u8])
                     -> Result<handle::Shader<Resources>, core::shade::CreateShaderError> {
        unimplemented!()
    }

    fn create_program(&mut self, _shader_set: &core::ShaderSet<Resources>)
                      -> Result<handle::Program<Resources>, core::shade::CreateProgramError> {
        unimplemented!()
    }

    fn create_pipeline_state_raw(&mut self, _program: &handle::Program<Resources>, _desc: &core::pso::Descriptor)
                                 -> Result<handle::RawPipelineState<Resources>, core::pso::CreationError> {
        unimplemented!()
    }

    fn create_texture_raw(&mut self, _desc: texture::Info, _hint: Option<format::ChannelType>, _data_opt: Option<&[&[u8]]>)
                          -> Result<handle::RawTexture<Resources>, texture::CreationError> {
        unimplemented!()
    }

    fn view_buffer_as_shader_resource_raw(&mut self, _hbuf: &handle::RawBuffer<Resources>)
                                      -> Result<handle::RawShaderResourceView<Resources>, core::factory::ResourceViewError> {
        unimplemented!()
    }

    fn view_buffer_as_unordered_access_raw(&mut self, _hbuf: &handle::RawBuffer<Resources>)
                                       -> Result<handle::RawUnorderedAccessView<Resources>, core::factory::ResourceViewError> {
        unimplemented!()
    }

    fn view_texture_as_shader_resource_raw(&mut self, _htex: &handle::RawTexture<Resources>, _desc: texture::ResourceDesc)
                                       -> Result<handle::RawShaderResourceView<Resources>, core::factory::ResourceViewError> {
        unimplemented!()
    }

    fn view_texture_as_unordered_access_raw(&mut self, _htex: &handle::RawTexture<Resources>)
                                        -> Result<handle::RawUnorderedAccessView<Resources>, core::factory::ResourceViewError> {
        unimplemented!()
    }

    fn view_texture_as_render_target_raw(&mut self, _htex: &handle::RawTexture<Resources>, _desc: texture::RenderDesc)
                                         -> Result<handle::RawRenderTargetView<Resources>, core::factory::TargetViewError> {
        unimplemented!()
    }

    fn view_texture_as_depth_stencil_raw(&mut self, _htex: &handle::RawTexture<Resources>, _desc: texture::DepthStencilDesc)
                                         -> Result<handle::RawDepthStencilView<Resources>, core::factory::TargetViewError> {
        unimplemented!()
    }

    fn create_sampler(&mut self, _info: texture::SamplerInfo) -> handle::Sampler<Resources> {
        unimplemented!()
    }

    fn map_buffer_raw(&mut self, _buf: &handle::RawBuffer<Resources>, _access: core::memory::Access)
                      -> Result<handle::RawMapping<Resources>, mapping::Error> {
        unimplemented!()
    }

    fn map_buffer_readable<T: Copy>(&mut self, _buf: &handle::Buffer<Resources, T>)
                                    -> Result<mapping::Readable<Resources, T>, mapping::Error> {
        unimplemented!()
    }

    fn map_buffer_writable<T: Copy>(&mut self, _buf: &handle::Buffer<Resources, T>)
                                    -> Result<mapping::Writable<Resources, T>, mapping::Error> {
        unimplemented!()
    }

    fn map_buffer_rw<T: Copy>(&mut self, buf: &handle::Buffer<Resources, T>)
                                    -> Result<mapping::RWable<Resources, T>, mapping::Error> {
        unimplemented!()
    }
}

pub enum CommandBuffer {
    OpenGL(gfx_device_gl::CommandBuffer),
    D3D11(gfx_device_dx11::CommandBuffer<gfx_device_dx11::DeferredContext>),
}

impl core::command::Buffer<Resources> for CommandBuffer {

}

pub enum Device {
    OpenGL(gfx_device_gl::Device),
    D3D11(gfx_device_dx11::Device),
}

impl core::Device for Device {
    type Resources = Resources;
    type CommandBuffer = CommandBuffer;

    fn get_capabilities(&self) -> &core::Capabilities {
        unimplemented!()
    }

    fn pin_submitted_resources(&mut self, handles: &handle::Manager<Self::Resources>) {
        unimplemented!()
    }

    fn submit(&mut self, buffer: &mut Self::CommandBuffer,
                         access: &pso::AccessInfo<Self::Resources>) {
        unimplemented!()
    }

    fn fenced_submit(&mut self,
                     buffer: &mut Self::CommandBuffer,
                     access: &pso::AccessInfo<Self::Resources>,
                     after: Option<handle::Fence<Self::Resources>>)
                     -> handle::Fence<Self::Resources> {
        unimplemented!()
    }

    fn cleanup(&mut self) {
        unimplemented!()
    }
}