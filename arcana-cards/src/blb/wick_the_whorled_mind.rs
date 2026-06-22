//! Wick, the Whorled Mind — `{3}{B}` 2/4 Legendary Creature — Rat Warlock.
//! Black.
//! "Whenever Wick or another Rat you control enters, create a 1/1 black Snail
//! creature token if you don't control a Snail. Otherwise, put a +1/+1 counter
//! on a Snail you control." — ZoneChange on Rats you control (Wick included);
//! resolved with a board-state branch on whether you control a Snail. The
//! "otherwise" counter-on-a-chosen-Snail branch has no target requirement and
//! no single-Snail selector, so it is GAP'd.
//! "{U}{B}{R}, Sacrifice a Snail: Wick deals damage equal to the sacrificed
//! creature's power to each opponent. Then draw cards equal to the sacrificed
//! creature's power." — sacrifice_other cost wired; the effect's amount
//! depends on the sacrificed creature's power, which the activation context
//! does not expose, so the payload is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wick, the Whorled Mind");
    let rat = reg.interner_mut().intern("Rat");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(warlock);
    // Pre-intern the Snail subtype so the resolver can rebuild the token + filter.
    let _snail = reg.interner_mut().intern("Snail");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let rat_filter = script::subtype_filter(reg, "Rat")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: rat_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: rat_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{B}{R}, Sacrifice a Snail: Wick deals damage equal to \
                       the sacrificed creature's power to each opponent. Then \
                       draw cards equal to the sacrificed creature's power."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{B}{R}").expect("valid cost"),
                    sacrifice_other: Some(script::subtype_filter(reg, "Snail")),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sacrificed_snail_payload,
            }),
    )
}

fn rat_enters(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let snail_filter = script::subtype_filter(reg, "Snail")
        .controlled_by(ControllerConstraint::You);
    if script::count_matching(state, &snail_filter, trig.controller) > 0 {
        // GAP: "Otherwise, put a +1/+1 counter on a Snail you control" — no
        // target requirement on this trigger and no single-Snail selector to
        // pick exactly one Snail; the counter branch is omitted.
        return Vec::new();
    }
    let snail = reg.interner().lookup("Snail").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snail);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: snail,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn sacrificed_snail_payload(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: damage / draw amount equals "the sacrificed creature's power", but
    // the activation context exposes no handle to the sacrificed-as-cost
    // object, so its power cannot be read at resolution. Emitting a literal
    // would be a materially wrong card, so the payload is omitted.
    Vec::new()
}
