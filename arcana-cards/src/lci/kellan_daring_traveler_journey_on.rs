//! Kellan, Daring Traveler // Journey On — `{1}{W}` Legendary Human Faerie
//! Scout 2/3 creature with an Adventure face "Journey On" (`{G}` Sorcery).
//!
//! Creature face:
//! - Whenever Kellan attacks, reveal the top card of your library. If it's a
//!   creature card with mana value 3 or less, put it into your hand. Otherwise,
//!   you may put it into your graveyard.
//!   Modeled as DigTopN with creature+max_cmc filter; rest goes to graveyard.
//!
//! Adventure face "Journey On" (`{G}` Sorcery):
//! - Create X Map tokens, where X is one plus the number of opponents who
//!   control an artifact.
//!   GAP: Map token type is not a CommodityToken variant and the Map
//!   activated ability (tap: this creature explores) is not expressible
//!   with the available API. Returning Vec::new().

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kellan, Daring Traveler");
    let human_sub = reg.interner_mut().intern("Human");
    let faerie_sub = reg.interner_mut().intern("Faerie");
    let scout_sub = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(faerie_sub);
    subtypes.0.insert(scout_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Adventure face "Journey On"
    let adv_name = reg.interner_mut().intern("Journey On");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create X Map tokens, where X is one plus the number of opponents who control an artifact.".into(),
        target_requirements: vec![],
        modal: None,
        effect: journey_on_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: kellan_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn kellan_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Reveal the top card. If it's a creature with mana value 3 or less, put
    // it into your hand; otherwise put the rest into your graveyard.
    let filter = Some(
        ObjectFilter::new()
            .with_types(TypeLine::CREATURE.into())
            .with_max_cmc(3),
    );
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter,
        rest: DigRest::Graveyard,
    }]
}

fn journey_on_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Map token type is not a CommodityToken variant and the Map
    // activated ability (tap: this creature explores) is not expressible
    // with the available API. Token count would be 1 + count of opponents
    // controlling an artifact (script::count_matching) but no Map token
    // creation primitive exists.
    Vec::new()
}
