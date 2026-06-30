//! Castle Locthwain — ELD utility land. "This land enters tapped unless you
//! control a Swamp" (`EntersWithSpec::TappedUnlessControl`), "{T}: Add {B}",
//! and "{1}{B}{B}, {T}: Draw a card, then you lose life equal to the number of
//! cards in your hand." The draw ability reads the post-draw hand size in its
//! effect fn to emit the dynamic life loss.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Castle Locthwain");
    let swamp = reg.interner_mut().intern("Swamp");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::TappedUnlessControl {
                filter: ObjectFilter::permanent().with_subtype_sym(swamp),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {B}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{B}, {T}: Draw a card, then you lose life equal to the number of cards in your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{B}").expect("valid cost"),
                    ..ActivationCost::tap_only()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: draw_and_lose,
            }),
    )
}

fn add_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn draw_and_lose(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let p = ctx.controller;
    // Post-draw hand size: current hand + 1 if the draw will actually happen.
    let will_draw = state.objects.count_in_zone(Zone::Library(p)) > 0;
    let post_draw_hand =
        state.objects.count_in_zone(Zone::Hand(p)) as u32 + u32::from(will_draw);
    vec![
        Effect::DrawCards { player: p, count: 1 },
        Effect::LoseLife { player: p, amount: post_draw_hand },
    ]
}
