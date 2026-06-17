//! Lae'zel, Illithid Thrall — `{3}{W}{U}` 3/6 Legendary Creature — Gith Horror Warrior.
//! Double strike.
//! When this creature enters or specializes, conjure a duplicate of a random
//!   creature card from an opponent's library into your hand. It perpetually
//!   gains "You may spend mana as though it were mana of any color to cast this
//!   spell."
//!
//! GAP: Conjure is not modeled (Arena-only mechanic; no Effect::Conjure
//! variant), and "perpetually gains" is not expressible — both trigger
//! effects return empty. The enters/specializes triggers are still wired so
//! the catalog records the trigger conditions.

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
    let name = reg.interner_mut().intern("Lae'zel, Illithid Thrall");
    let gith = reg.interner_mut().intern("Gith");
    let horror = reg.interner_mut().intern("Horror");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gith);
    subtypes.0.insert(horror);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: conjure_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: conjure_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn conjure_gap(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name
    // lookup in Effect::execute) + perpetual mana-any-color grant inexpressible.
    Vec::new()
}
