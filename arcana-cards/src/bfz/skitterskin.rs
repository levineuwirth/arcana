//! Skitterskin — `{3}{B}` 4/3 Eldrazi Drone with Devoid (it has no color).
//! "This creature can't block" (a static) is GAP'd.
//! "{1}{B}: Regenerate this creature. Activate only if you control another
//! colorless creature." — the regenerate + the activation gate are wired;
//! "another" is approximated by requiring at least two colorless creatures
//! you control (this creature counts as one).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skitterskin");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        // Devoid — this card has no color.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "This creature can't block" — no Effect/keyword for a
    // permanent self can't-block static here.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}: Regenerate this creature. Activate only if you control another colorless creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                activation_condition: Some(control_another_colorless),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: regen_self,
        }),
    )
}

fn colorless_creature_filter() -> ObjectFilter {
    // A creature with none of the five colors present (== colorless).
    ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .without_colors(
            ColorSet::white()
                | ColorSet::blue()
                | ColorSet::black()
                | ColorSet::red()
                | ColorSet::green(),
        )
}

fn control_another_colorless(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "another colorless creature" — approximated as >= 2 colorless
    // creatures you control (this source counts as one).
    conditions::you_control_at_least(s, you, &colorless_creature_filter(), 2)
}

fn regen_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
