//! Momo, Friendly Flier — `{W}` 1/1 Legendary Lemur Bat Ally with Flying.
//!
//! Oracle:
//! * Flying
//! * The first non-Lemur creature spell with flying you cast during each of
//!   your turns costs {1} less to cast.
//! * Whenever another creature you control with flying enters, Momo gets
//!   +1/+1 until end of turn.
//!
//! Decomposition:
//! 1. Keyword line: Flying.
//! 2. Cost-reduction static — GAP: no cost-reduction Effect / static is
//!    available to this card class.
//! 3. ZoneChange trigger (a flying creature you control enters) → Momo gets
//!    +1/+1 until end of turn. The "another" is naturally satisfied: when this
//!    trigger resolves, Momo has already entered and isn't the entering object.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Momo, Friendly Flier");
    let lemur = reg.interner_mut().intern("Lemur");
    let bat = reg.interner_mut().intern("Bat");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lemur);
    subtypes.0.insert(bat);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "The first non-Lemur creature spell with flying you cast
            // during each of your turns costs {1} less" — no cost-reduction
            // static expressible for this card class.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_keyword(KeywordAbility::Flying),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: pump_momo,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_momo(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
