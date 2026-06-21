//! Myriad Construct — `{4}` 4/4 Artifact Creature — Construct.
//! Kicker {3}.
//! If this creature was kicked, it enters with a +1/+1 counter on it for
//! each nonbasic land your opponents control.
//! When this creature becomes the target of a spell, sacrifice it and
//! create a number of 1/1 colorless Construct artifact creature tokens
//! equal to its power.
//!
//! Bones + the becomes-target trigger are wired. Kicker (and the
//! kicked-dependent ETB counters) are GAP'd — Kicker is not in the usable
//! keyword surface and the optional-kick cast cost / kicked-state ETB rider
//! are not expressible. The becomes-target trigger: power is captured at
//! resolution, the source is sacrificed (DestroyPermanent self, the
//! catalog's Phase-1 sacrifice approximation), then that-many 1/1 Construct
//! tokens are minted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myriad Construct");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Kicker {3} — not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: "If this creature was kicked, it enters with a +1/+1 counter for
    // each nonbasic land your opponents control." — kicked-state ETB rider.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: sacrifice_make_constructs,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sacrifice_make_constructs(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::power_of(state, trig.source).max(0) as u32;
    let construct = reg.interner().lookup("Construct").unwrap_or_default();

    let mut effects: Vec<Effect> = Vec::with_capacity(1 + n as usize);
    // Sacrifice it (Phase-1 self-sacrifice routes through DestroyPermanent).
    effects.push(Effect::DestroyPermanent { target: trig.source });
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(construct);
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: construct,
                colors: ColorSet::colorless(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
