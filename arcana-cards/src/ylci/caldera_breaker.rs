//! Caldera Breaker — `{3}{R}{R}{R}` 6/6 red Artifact Creature — Golem with Trample.
//! ETB ("exile all Mountain cards from your library; when you do, deal that
//! much damage to a target …") combines a library-wide exile with a reflexive
//! damage trigger keyed to the exiled count — not expressible as one Effect, so
//! the trigger body is GAP'd. The dies trigger ("return all cards exiled with it
//! to the battlefield; Conjure four Volcanic Geyser") relies on exiled-with-it
//! linkage + Conjure (Arena-only, not modeled), also GAP'd. Trample is expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caldera Breaker");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_mountains,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_return_exiled,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile_mountains(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile all Mountain cards from your library; when you do, this deals
    //       that much damage to a target creature/planeswalker an opponent
    //       controls" — library-wide exile feeding a reflexive count-scaled
    //       damage trigger; no single Effect expresses the exile-all-from-library
    //       + reflexive-target combination.
    Vec::new()
}

fn dies_return_exiled(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put all cards exiled with it onto the battlefield" needs exiled-with-it
    //       linkage tracking; "Conjure four cards named Volcanic Geyser" is Conjure
    //       (Arena-only, no Effect::Conjure variant).
    Vec::new()
}
