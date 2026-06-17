//! Durnan of the Yawning Portal — `{3}{G}` 3/3 Legendary Human Warrior.
//! "Whenever Durnan attacks, look at the top four cards of your library. You may
//! exile a creature card from among them. Put the rest on the bottom of your
//! library in any order. For as long as that card remains exiled, you may cast
//! it. That spell has undaunted."
//! "Choose a Background." (deck-building keyword — not an available KeywordAbility)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Durnan of the Yawning Portal");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Choose a Background" is a deck-building keyword, not a KeywordAbility.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: durnan_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn durnan_dig(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top four, you may exile a creature card; for as long as
    // that card remains exiled you may cast it; that spell has undaunted." The
    // exile-with-ongoing-cast-permission-plus-undaunted is not expressible
    // (ImpulseExile exiles a fixed top-N for one turn; DigTopN goes to hand).
    Vec::new()
}
