//! Stone-Tongue Basilisk — `{4}{G}{G}{G}` 4/5 Basilisk. (Threshold is not a usable
//! KeywordAbility variant.)
//! "Whenever this creature deals combat damage to a creature, destroy that creature
//! at end of combat."
//! "Threshold — As long as there are seven or more cards in your graveyard, all
//! creatures able to block this creature do so."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stone-Tongue Basilisk");
    let basilisk = reg.interner_mut().intern("Basilisk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(basilisk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Threshold is not an available KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Creature,
                combat_only: true,
            },
            intervening_if: None,
            effect: destroy_damaged_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: Threshold static "all creatures able to block this creature do so" is a
    // conditional continuous static (lure-while-threshold), not a triggered/
    // activated ability — not expressible with the demonstrated API.
}

fn destroy_damaged_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no PendingTrigger accessor exposes the damaged CREATURE's ObjectId for a
    // DamageDealt trigger (only damaged_player), and there is no "at end of combat"
    // DelayedWhen window — so "destroy that creature at end of combat" cannot be
    // expressed.
    Vec::new()
}
