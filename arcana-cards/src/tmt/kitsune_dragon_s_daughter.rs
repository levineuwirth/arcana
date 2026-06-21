//! Kitsune, Dragon's Daughter — `{4}{U}{U}` 6/6 Legendary Fox Warlock Avatar
//! with Vigilance.
//!
//! "Whenever Kitsune enters or deals combat damage to a player, you may
//! exchange control of two other target creatures controlled by different
//! players."
//!
//! The enters / combat-damage-to-a-player pair becomes two triggered
//! abilities. The payload — exchanging control of two creatures controlled by
//! different players — has no documented Effect representation (ChangeControl
//! moves one creature to a single controller; there is no exchange/swap
//! primitive), so the effect is GAP'd. The trigger structure and Vigilance
//! keyword are recorded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kitsune, Dragon's Daughter");
    let fox = reg.interner_mut().intern("Fox");
    let warlock = reg.interner_mut().intern("Warlock");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(warlock);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exchange_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: exchange_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exchange_control(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may exchange control of two other target creatures controlled
    // by different players" — no documented exchange/swap-control effect
    // (ChangeControl only moves a single creature to one controller).
    Vec::new()
}
