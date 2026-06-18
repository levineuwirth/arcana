//! Ezio Auditore da Firenze — `{1}{B}` 3/2 Legendary Creature — Human
//! Assassin with Menace.
//!
//! "Assassin spells you cast have freerunning {B}{B}.
//!  Whenever Ezio deals combat damage to a player, you may pay {W}{U}{B}{R}{G}
//!  if that player has 10 or less life. When you do, that player loses the
//!  game."
//!
//! Menace is a base keyword. The freerunning grant is an alternative-cost
//! static that isn't modeled — GAP'd. The combat-damage trigger fires (self
//! deals combat damage to a player), but its payload (gate on the damaged
//! player's life, an optional WUBRG payment, then "that player loses the
//! game") has no "loses the game" primitive, so the payload is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ezio Auditore da Firenze");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    let self_filter = arcana_core::targets::ObjectFilter {
        name: Some(name),
        ..arcana_core::targets::ObjectFilter::default()
    };

    // GAP (static): "Assassin spells you cast have freerunning {B}{B}" —
    //      freerunning alternative cost is not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_finisher,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_finisher(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {W}{U}{B}{R}{G} if that player has 10 or less life.
    // When you do, that player loses the game." No "loses the game" primitive.
    Vec::new()
}
