//! Guardian Sunmare — `{3}{W}{W}` 5/5 Horse Mount.
//!
//! * Ward {2}.
//! * Whenever this creature attacks while saddled, search your library
//!   for a nonland permanent card with mana value 3 or less, put it onto
//!   the battlefield, then shuffle.
//! * Saddle 4.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guardian Sunmare");
    let horse = reg.interner_mut().intern("Horse");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: Saddle 4 — the Saddle keyword / activated saddle cost has no
    // KeywordAbility variant or ActivationCost shape in the catalog, so
    // the saddle mechanic itself is not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "while saddled" gate is not expressible (no saddled
            // status accessor / intervening-if predicate); the attack
            // trigger fires on every attack rather than only when saddled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: tutor_nonland_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tutor_nonland_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent()
            .without_types(TypeLine::LAND.into())
            .with_max_cmc(3),
        tapped: false,
    }]
}
