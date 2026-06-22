//! Heart Wolf — `{3}{R}` 2/2 Creature — Wolf.
//! First strike.
//! {T}: Target Dwarf creature gets +2/+0 and gains first strike until end of
//! turn. When that creature leaves the battlefield this turn, sacrifice this
//! creature. Activate only during combat.
//!
//! Decomposition:
//! 1. Keyword line: First strike.
//! 2. {T}: target Dwarf creature gets +2/+0 and gains first strike until end of
//!    turn (single Effect::Pump granting the keyword).
//!    GAPs within this ability:
//!      • "When that creature leaves the battlefield this turn, sacrifice this
//!        creature." — a delayed trigger that WATCHES the chosen target but
//!        SACRIFICES the source (Heart Wolf). DelayedAction::Sacrifice operates
//!        on its own `source`, so this cross-object link (watch target → kill a
//!        different object) is not expressible. Omitted.
//!      • "Activate only during combat." — no combat-window activation-timing
//!        field in the cost catalog. Omitted (the ability is sorcery-speed by
//!        default, which is a fidelity gap on the timing window).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heart Wolf");
    let wolf = reg.interner_mut().intern("Wolf");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target Dwarf creature gets +2/+0 and gains first strike until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_subtype_sym(dwarf),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_dwarf,
            }),
    )
}

fn pump_dwarf(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike],
    }]
}
