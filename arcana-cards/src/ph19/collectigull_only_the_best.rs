//! Collectigull // Only the Best — `{2}{W}` // `{1}{W}` white Adventure creature.
//! Creature: 1/1 Bird. Flying, protection from common (not in keyword catalog).
//! Whenever Collectigull attacks, reveal top card of library. If it's a Booster Fun card, put it in hand.
//! Adventure (Only the Best — Sorcery): Return target Booster Fun card from graveyard to hand.
//! GAP: "protection from common" — Protection keyword with rarity target not in catalog.
//! GAP: "Booster Fun card" — rarity/treatment filter not in ObjectFilter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Collectigull");
    let adv_name = reg.interner_mut().intern("Only the Best");
    let bird_sub = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "protection from common" not in KeywordAbility catalog
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Return target Booster Fun card from your graveyard to your hand.".into(),
        target_requirements: vec![],
        modal: None,
        effect: only_the_best_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: collectigull_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn collectigull_attacks(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal top card of library; if Booster Fun card, put in hand" — Booster Fun filter not in catalog
    Vec::new()
}

fn only_the_best_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target Booster Fun card from graveyard" — Booster Fun filter not in ObjectFilter
    Vec::new()
}
