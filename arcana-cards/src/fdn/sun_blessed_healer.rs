//! Sun-Blessed Healer — `{1}{W}` 3/1 Human Cleric with Lifelink.
//!
//! Oracle:
//! * Kicker {1}{W}.
//! * Lifelink.
//! * When this creature enters, if it was kicked, return target nonland
//!   permanent card with mana value 2 or less from your graveyard to the
//!   battlefield.
//!
//! GAP: Kicker is not in the usable keyword surface for this card class — the
//! {1}{W} kicker cost and the "if it was kicked" intervening-if (kicked status
//! is not an available `conditions::` predicate) are not expressible. The ETB
//! reanimation effect is wired UNCONDITIONALLY (the kicked gate is dropped).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sun-Blessed Healer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if it was kicked" — kicked status is not an available
                // intervening-if predicate; effect fires unconditionally.
                intervening_if: None,
                effect: etb_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into())
                            .with_max_cmc(2)
                            .controlled_by(ControllerConstraint::You),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
