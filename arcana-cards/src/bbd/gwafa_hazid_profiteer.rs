//! Gwafa Hazid, Profiteer — `{1}{W}{U}` 2/2 Legendary Human Rogue (white/blue).
//! "{W}{U}, {T}: Put a bribery counter on target creature you don't control.
//!  Its controller draws a card.
//!  Creatures with bribery counters on them can't attack or block."
//!
//! The activated ability puts a "bribery" named counter on a target creature
//! you don't control, then that creature's controller draws a card. The static
//! "can't attack or block while bribery-countered" line has no expressible
//! hook → GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gwafa Hazid, Profiteer");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    // Pre-intern the named counter so the resolver can recover it.
    let _bribery = reg.interner_mut().intern("bribery");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}{U}, {T}: Put a bribery counter on target creature you don't control. Its controller draws a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}{U}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: bribery_counter,
        }),
        // GAP: "Creatures with bribery counters on them can't attack or block."
        // — static counter-gated combat restriction, not expressible.
    )
}

fn bribery_counter(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let kind = match reg.interner().lookup("bribery") {
        Some(sym) => CounterKind::Named(sym),
        None => return Vec::new(),
    };
    let controller = script::target_controller(state, *id, ctx.controller);
    vec![
        Effect::AddCounters {
            target: *id,
            kind,
            count: 1,
        },
        Effect::DrawCards {
            player: controller,
            count: 1,
        },
    ]
}
