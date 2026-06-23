//! Aquastrand Spider — `{1}{G}` 0/0 Spider Mutant.
//! Graft 2 (enters with two +1/+1 counters; whenever another creature
//! enters you may move a +1/+1 counter from this onto it — the Graft
//! keyword machinery handles both halves).
//! `{G}: Target creature with a +1/+1 counter on it gains reach until
//! end of turn.`

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
    let name = reg.interner_mut().intern("Aquastrand Spider");
    let spider = reg.interner_mut().intern("Spider");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Graft(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{G}: Target creature with a +1/+1 counter on it gains reach until end of turn."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter {
                    has_counter: Some(CounterKind::PlusOnePlusOne),
                    ..ObjectFilter::creature()
                }),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_reach,
        }),
    )
}

fn grant_reach(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Reach,
        duration: Duration::EndOfTurn,
    }]
}
