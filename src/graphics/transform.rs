/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! Define a 3x3 transform matrix.
//!
//! A Transform specifies how to translate,
//! rotate, scale, shear, project, whatever things.

use libc::c_float;

use sfml_types::Vector2f;
use graphics::FloatRect;

use csfml_graphics_sys as ffi;

/// Define a 3x3 transform matrix.
///
/// A sfTransform specifies how to translate,
/// rotate, scale, shear, project, whatever things.
#[repr(C)]
pub struct Transform(pub ffi::sfTransform);

impl Transform {
    /// Create a new transform from a matrix
    ///
    /// # Arguments
    /// * a00 - Element (0, 0) of the matrix
    /// * a01 - Element (0, 1) of the matrix
    /// * a02 - Element (0, 2) of the matrix
    /// * a10 - Element (1, 0) of the matrix
    /// * a11 - Element (1, 1) of the matrix
    /// * a12 - Element (1, 2) of the matrix
    /// * a20 - Element (2, 0) of the matrix
    /// * a21 - Element (2, 1) of the matrix
    /// * a22 - Element (2, 2) of the matrix
    ///
    /// Return a new Transform
    pub fn new(a00: f32, a01: f32, a02: f32,
               b10: f32, b11: f32, b12: f32,
               c20: f32, c21: f32, c22: f32) -> Transform {
        unsafe {
            Transform(ffi::sfTransform_fromMatrix(a00, a01, a02,
                                        b10, b11, b12,
                                        c20, c21, c22))
        }
    }

    /// Return the matrix
    pub fn get_matrix(&mut self) -> [f32; 16] {
        unsafe {
            let matrix: [f32; 16] =
                [0.,0.,0.,0.,
                 0.,0.,0.,0.,
                 0.,0.,0.,0.,
                 0.,0.,0.,0.];
            ffi::sfTransform_getMatrix(&mut self.0, matrix.as_ptr() as *mut f32);
            matrix
        }
    }

    /// Create a new identity transform
    ///
    /// Return a new Transform initialized at 1, 0, 0, 0, 1, 0, 0, 0, 1
    pub fn new_identity() -> Transform {
        unsafe {
            Transform(ffi::sfTransform_fromMatrix(1., 0., 0., 0., 1., 0., 0., 0., 1.))
        }
    }

    /// Return the inverse of a transform
    ///
    /// If the inverse cannot be computed, a new identity transform
    /// is returned.
    ///
    /// Return the inverse matrix
    pub fn get_inverse(&mut self) -> Transform {
        unsafe {
            Transform(ffi::sfTransform_getInverse(&mut self.0))
        }
    }

    /// Combine two transforms
    ///
    /// The result is a transform that is equivalent to applying
    /// transform followed by other. Mathematically, it is
    /// equivalent to a matrix multiplication.
    ///
    /// # Arguments
    /// * other - Transform to combine to transform
    pub fn combine(&mut self, other: &mut Transform) {
        unsafe {
            ffi::sfTransform_combine(&mut self.0, &mut other.0)
        }
    }

    /// Combine a transform with a translation
    ///
    /// # Arguments
    /// * x - Offset to apply on X axis
    /// * y - Offset to apply on Y axis
    pub fn translate(&mut self, x: f32, y: f32) {
        unsafe {
            ffi::sfTransform_translate(&mut self.0, x as c_float, y as c_float)
        }
    }

    /// Combine the current transform with a rotation
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    pub fn rotate(&mut self, angle: f32) {
        unsafe {
            ffi::sfTransform_rotate(&mut self.0, angle as c_float)
        }
    }

    /// Combine the current transform with a rotation
    ///
    /// The center of rotation is provided for convenience as a second
    /// argument, so that you can build rotations around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), rotate(angle), translate(center)].
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    /// * center_x - X coordinate of the center of rotation
    /// * center_y - Y coordinate of the center of rotation
    pub fn rotate_with_center(&mut self,
                              angle: f32,
                              center_x: f32,
                              center_y: f32) {
        unsafe {
            ffi::sfTransform_rotateWithCenter(&mut self.0,
                                              angle as c_float,
                                              center_x as c_float,
                                              center_y as c_float)
        }
    }

    /// Combine the current transform with a scaling
    ///
    /// # Arguments
    /// * scale_x - Scaling factor on the X axis
    /// * scale_y - Scaling factor on the Y axis
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfTransform_scale(&mut self.0, scale_x as c_float, scale_y as c_float)
        }
    }

    /// Combine the current transform with a scaling
    ///
    /// The center of scaling is provided for convenience as a second
    /// argument, so that you can build scaling around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), scale(factors), translate(center)]
    ///
    /// # Arguments
    /// * scale_x - Scaling factor on X axis
    /// * scale_y - Scaling factor on Y axis
    /// * center_x - X coordinate of the center of scaling
    /// * center_y - Y coordinate of the center of scaling
    pub fn scale_with_center(&mut self,
                             scale_x: f32,
                             scale_y: f32,
                             center_x: f32,
                             center_y: f32) {
        unsafe {
            ffi::sfTransform_scaleWithCenter(&mut self.0,
                                             scale_x,
                                             scale_y,
                                             center_x,
                                             center_y)
        }
    }

    /// Apply a transform to a 2D point
    ///
    /// # Arguments
    /// * point - Point to transform
    ///
    /// Return a transformed point
    pub fn transform_point(&mut self, point: &Vector2f) -> Vector2f {
        unsafe {
            ffi::sfTransform_transformPoint(&mut self.0, *point)
        }
    }

    /// Apply a transform to a rectangle
    ///
    /// Since SFML doesn't provide support for oriented rectangles,
    /// the result of this function is always an axis-aligned
    /// rectangle. Which means that if the transform contains a
    /// rotation, the bounding rectangle of the transformed rectangle
    /// is returned.
    ///
    /// # Arguments
    /// rectangle - Rectangle to transform
    ///
    /// Return the transformed rectangle
    pub fn transform_rect(&mut self, rectangle: &FloatRect) -> FloatRect {
        unsafe {
            ffi::sfTransform_transformRect(&mut self.0, *rectangle)
        }
    }
}
