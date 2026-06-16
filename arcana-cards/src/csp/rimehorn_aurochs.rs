//! Rimehorn Aurochs — `{4}{G}` 3/3 Snow Creature — Aurochs with Trample.
//! "Whenever this creature attacks, it gets +1/+0 until end of turn for each
//! other attacking Aurochs."
//! "{2}{S}: Target creature blocks target creature this turn if able."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rimehorn Aurochs");
    let aurochs = reg.interner_mut().intern("Aurochs");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aurochs);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_per_attacking_aurochs,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{S}: Target creature blocks target creature this turn if able.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: must_block,
            }),
    )
}

fn pump_per_attacking_aurochs(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: amount = "each OTHER ATTACKING Aurochs" — no script helper counts
    // attacking creatures of a subtype (excluding self); cannot compute the
    // dynamic +N/+0, and hardcoding would be materially wrong.
    Vec::new()
}

fn must_block(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Target creature blocks target creature this turn if able" — no
    // forced-block / lure Effect variant is available; ForbidBlocking is the
    // opposite. Targets are declared but the effect is unexpressible.
    Vec::new()
}
