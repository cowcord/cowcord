use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
	#[default]
	Default,
	Destructive,
	Outline,
	Secondary,
	Ghost,
	Link,
}

impl ButtonVariant {
	pub const fn class(&self) -> &'static str {
		match self {
			| Self::Default => "bg-primary text-primary-foreground shadow-xs hover:bg-primary/80",
			| Self::Destructive => {
				"bg-destructive shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 text-destructive-foreground"
			},
			| Self::Outline => {
				"bg-background shadow-xs hover:bg-accent hover:text-accent-foreground border border-border"
			},
			| Self::Secondary => {
				"bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80"
			},
			| Self::Ghost => "hover:bg-accent hover:text-accent-foreground",
			| Self::Link => "text-primary underline-offset-4 hover:underline",
		}
	}
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
	#[default]
	Default,
	Sm,
	Lg,
	Icon,
	IconSm,
	IconLg,
}

impl ButtonSize {
	pub const fn class(&self) -> &'static str {
		match self {
			| Self::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
			| Self::Sm => "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5",
			| Self::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
			| Self::Icon => "size-9",
			| Self::IconSm => "size-8",
			| Self::IconLg => "size-10",
		}
	}
}

const BASE_CLASS: &str = "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium outline-none transition-all focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0";

#[component]
pub fn Button(
	children: Element,
	#[props(default)] variant: ButtonVariant,
	#[props(default)] size: ButtonSize,
	#[props(into, default)] class: String,
	href: Option<String>,
	#[props(default = "button".to_string())] button_type: String,
	#[props(default = false)] disabled: bool,
	onclick: Option<EventHandler<MouseEvent>>,
	#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
	let class = tw_merge!(
		class,
		format!("{} {} {}", BASE_CLASS, variant.class(), size.class())
	);

	println!("{}", class);

	if let Some(href) = href {
		rsx! {
			a {
				"data-slot": "button",
				class,
				href: if !disabled { href } else { "".to_string() },
				"aria-disabled": "{disabled}",
				role: if disabled { "link" } else { "" },
				tabindex: if disabled { -1 } else { 0 },
				onclick: move |evt| {
					if let Some(handler) = &onclick {
						handler.call(evt);
					}
				},
				..attributes,
				{children}
			}
		}
	} else {
		rsx! {
			button {
				"data-slot": "button",
				class,
				r#type: "{button_type}",
				disabled,
				onclick: move |evt| {
					if let Some(handler) = &onclick {
						handler.call(evt);
					}
				},
				..attributes,
				{children}
			}
		}
	}
}
