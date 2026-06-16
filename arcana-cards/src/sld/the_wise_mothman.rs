//! The Wise Mothman — `{1}{B}{G}{U}` 3/3 Legendary Insect Mutant. Flying.
//! "Whenever The Wise Mothman enters or attacks, each player gets a rad counter."
//!  — split into an enters trigger and an attacks trigger; rad counters go on
//!  PLAYERS, but AddCounters targets an ObjectId only (no player-counter primitive),
//!  so the effects are GAP'd.
//! "Whenever one or more nonland cards are milled, put a +1/+1 counter on each of up
//!  to X target creatures, where X is the number of nonland cards milled this way."
//!  — no 'cards milled' TriggerCondition variant; GAP'd entirely.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Wise Mothman");
    let insect = reg.interner_mut().intern("Insect");
    let mutant = reg.interner_mut().intern("Mutant");
    let _rad = reg.interner_mut().intern("rad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "whenever one or more nonland cards are milled, +1/+1 counters on up to X
    //   target creatures" — no 'cards milled' TriggerCondition variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: each_player_rad_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: each_player_rad_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_player_rad_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player gets a rad counter" — rad counters are placed on PLAYERS, but
    // AddCounters targets an ObjectId; no player-counter primitive is demonstrated.
    Vec::new()
}
