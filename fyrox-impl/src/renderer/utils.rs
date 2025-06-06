// Copyright (c) 2019-present Dmitry Stepanov and Fyrox Engine contributors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use fyrox_core::algebra::Vector3;
use fyrox_graphics::gpu_texture::CubeMapFace;

pub struct CubeMapFaceDescriptor {
    pub face: CubeMapFace,
    pub look: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl CubeMapFaceDescriptor {
    pub fn cube_faces() -> [Self; 6] {
        [
            CubeMapFaceDescriptor {
                face: CubeMapFace::PositiveX,
                look: Vector3::new(1.0, 0.0, 0.0),
                up: Vector3::new(0.0, -1.0, 0.0),
            },
            CubeMapFaceDescriptor {
                face: CubeMapFace::NegativeX,
                look: Vector3::new(-1.0, 0.0, 0.0),
                up: Vector3::new(0.0, -1.0, 0.0),
            },
            CubeMapFaceDescriptor {
                face: CubeMapFace::PositiveY,
                look: Vector3::new(0.0, 1.0, 0.0),
                up: Vector3::new(0.0, 0.0, 1.0),
            },
            CubeMapFaceDescriptor {
                face: CubeMapFace::NegativeY,
                look: Vector3::new(0.0, -1.0, 0.0),
                up: Vector3::new(0.0, 0.0, -1.0),
            },
            CubeMapFaceDescriptor {
                face: CubeMapFace::PositiveZ,
                look: Vector3::new(0.0, 0.0, 1.0),
                up: Vector3::new(0.0, -1.0, 0.0),
            },
            CubeMapFaceDescriptor {
                face: CubeMapFace::NegativeZ,
                look: Vector3::new(0.0, 0.0, -1.0),
                up: Vector3::new(0.0, -1.0, 0.0),
            },
        ]
    }
}
