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

use crate::fyrox::{
    asset::{
        graph::{ResourceDependencyGraph, ResourceGraphNode},
        untyped::UntypedResource,
    },
    core::{log::Log, pool::Handle},
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        copypasta::ClipboardProvider,
        grid::{Column, GridBuilder, Row},
        message::{MessageDirection, UiMessage},
        scroll_viewer::ScrollViewerBuilder,
        stack_panel::StackPanelBuilder,
        text::TextBuilder,
        tree::{TreeBuilder, TreeRootBuilder, TreeRootMessage},
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowMessage, WindowTitle},
        BuildContext, HorizontalAlignment, Orientation, Thickness, UiNode, UserInterface,
        VerticalAlignment,
    },
};
use fyrox::asset::manager::ResourceManager;

pub struct DependencyViewer {
    pub window: Handle<UiNode>,
    tree_root: Handle<UiNode>,
    close: Handle<UiNode>,
    copy_to_clipboard: Handle<UiNode>,
    resource_graph: Option<ResourceDependencyGraph>,
}

fn build_tree_recursively(
    node: &ResourceGraphNode,
    resource_manager: &ResourceManager,
    ctx: &mut BuildContext,
) -> Handle<UiNode> {
    let children = node
        .children
        .iter()
        .map(|c| build_tree_recursively(c, resource_manager, ctx))
        .collect();

    let data_type = node.resource.data_type_name_or_unknown();

    let name = resource_manager
        .resource_path(&node.resource)
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|| "Embedded".to_string());

    TreeBuilder::new(WidgetBuilder::new())
        .with_items(children)
        .with_content(
            TextBuilder::new(
                WidgetBuilder::new().with_vertical_alignment(VerticalAlignment::Center),
            )
            .with_text(format!("{name} ({data_type})"))
            .build(ctx),
        )
        .build(ctx)
}

impl DependencyViewer {
    pub fn new(ctx: &mut BuildContext) -> Self {
        let tree_root;
        let copy_to_clipboard;
        let close;
        let window = WindowBuilder::new(WidgetBuilder::new().with_width(300.0).with_height(400.0))
            .open(false)
            .with_title(WindowTitle::text("Dependency Viewer"))
            .with_content(
                GridBuilder::new(
                    WidgetBuilder::new()
                        .with_child(
                            ScrollViewerBuilder::new(WidgetBuilder::new().on_row(0))
                                .with_content({
                                    tree_root = TreeRootBuilder::new(
                                        WidgetBuilder::new().with_margin(Thickness::uniform(1.0)),
                                    )
                                    .build(ctx);
                                    tree_root
                                })
                                .build(ctx),
                        )
                        .with_child(
                            StackPanelBuilder::new(
                                WidgetBuilder::new()
                                    .with_margin(Thickness::uniform(2.0))
                                    .with_horizontal_alignment(HorizontalAlignment::Right)
                                    .on_row(1)
                                    .with_child({
                                        copy_to_clipboard = ButtonBuilder::new(
                                            WidgetBuilder::new()
                                                .with_width(130.0)
                                                .with_margin(Thickness::uniform(1.0)),
                                        )
                                        .with_text("Copy To Clipboard")
                                        .build(ctx);
                                        copy_to_clipboard
                                    })
                                    .with_child({
                                        close = ButtonBuilder::new(
                                            WidgetBuilder::new()
                                                .with_width(130.0)
                                                .with_margin(Thickness::uniform(1.0)),
                                        )
                                        .with_text("Close")
                                        .build(ctx);
                                        close
                                    }),
                            )
                            .with_orientation(Orientation::Horizontal)
                            .build(ctx),
                        ),
                )
                .add_row(Row::stretch())
                .add_row(Row::strict(24.0))
                .add_column(Column::stretch())
                .build(ctx),
            )
            .build(ctx);

        Self {
            window,
            tree_root,
            copy_to_clipboard,
            close,
            resource_graph: None,
        }
    }

    pub fn open(
        &mut self,
        resource: &UntypedResource,
        resource_manager: &ResourceManager,
        ui: &mut UserInterface,
    ) {
        let resource_graph = ResourceDependencyGraph::new(resource);
        let root =
            build_tree_recursively(&resource_graph.root, resource_manager, &mut ui.build_ctx());
        ui.send_message(TreeRootMessage::items(
            self.tree_root,
            MessageDirection::ToWidget,
            vec![root],
        ));
        ui.send_message(WindowMessage::open(
            self.window,
            MessageDirection::ToWidget,
            true,
            true,
        ));
        self.resource_graph = Some(resource_graph);
    }

    pub fn handle_ui_message(&mut self, message: &UiMessage, ui: &mut UserInterface) {
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == self.close {
                ui.send_message(WindowMessage::close(
                    self.window,
                    MessageDirection::ToWidget,
                ));
            } else if message.destination() == self.copy_to_clipboard {
                if let Some(mut clipboard) = ui.clipboard_mut() {
                    if let Some(resource_graph) = self.resource_graph.as_ref() {
                        Log::verify(clipboard.set_contents(resource_graph.pretty_print()));
                    }
                }
            }
        } else if let Some(WindowMessage::Close) = message.data() {
            self.resource_graph = None;
        }
    }
}
