//! Tin-Wing Chimera — `{4}` 2/2 Artifact Creature — Chimera.
//! Flying.
//! "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature.
//! It gains flying. (This effect lasts indefinitely.)"
//!
//! The "+2/+2 counter" is modeled as two +1/+1 counters (same net boost; the
//! engine has no dedicated +2/+2 counter kind), and the indefinite flying grant
//! uses Duration::Permanent.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tin-Wing Chimera");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);

    let chimera_filter = ObjectFilter::creature().with_subtype_sym(chimera);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice this creature: Put a +2/+2 counter on target Chimera creature. It gains flying.".into(),
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

fn buff_chimera(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Flying,
            duration: Duration::Permanent,
        },
    ]
}
