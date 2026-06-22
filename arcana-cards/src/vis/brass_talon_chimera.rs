//! Brass-Talon Chimera — `{4}` 2/2 colorless Artifact Creature — Chimera.
//! First strike.
//! Sacrifice this creature: Put a +2/+2 counter on target Chimera creature.
//! It gains first strike. (This effect lasts indefinitely.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brass-Talon Chimera");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    let chimera_filter = script::subtype_filter(reg, "Chimera");
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature. It gains first strike.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(chimera_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: buff_chimera,
            }),
    )
}

fn buff_chimera(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // A +2/+2 counter is modeled as two +1/+1 counters (no PlusTwoPlusTwo
    // CounterKind exists; functionally equivalent P/T). The "gains first
    // strike indefinitely" grant is a permanent keyword grant.
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::Permanent,
        },
    ]
}
