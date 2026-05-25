//! Yore-Tiller Nephilim — `{W}{U}{B}{R}` 2/2 white-blue-black-red Nephilim.
//! "Whenever this creature attacks, return target creature card from
//! your graveyard to the battlefield tapped and attacking."
//! GAP: returning tapped and attacking — ReturnFromGraveyardToBattlefield
//! doesn't support tapped/attacking modifier; emitting plain return.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yore-Tiller Nephilim");
    let nephilim = reg.interner_mut().intern("Nephilim");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nephilim);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn attack_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "tapped and attacking" modifier on return — not supported by
    // ReturnFromGraveyardToBattlefield
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
