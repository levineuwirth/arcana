//! Mischievous Catgeist // Catlike Curiosity
//!
//! Front (Creature — Cat Spirit 1/1, {1}{U}):
//!   Whenever this creature deals combat damage to a player, draw a card.
//!   Disturb {2}{U} (cast from graveyard transformed).
//! Back (Enchantment — Aura):
//!   Enchant creature.
//!   Enchanted creature has "Whenever this creature deals combat damage to a player, draw a card."
//!   If Catlike Curiosity would be put into a graveyard from anywhere, exile it instead.
//!
//! GAP: Disturb cast-from-graveyard mechanic not modeled (engine debt).
//! GAP: back-face Aura granting an ability to enchanted creature not modeled (static layer).
//! GAP: "if would be put into a graveyard, exile instead" replacement is engine debt.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mischievous Catgeist");
    let cat = reg.interner_mut().intern("Cat");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Catlike Curiosity");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    // Front-face trigger: whenever this deals combat damage to a player, draw a card.
    let damage_trig = TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::DamageDealt {
            source_filter: ObjectFilter::new(),
            target_filter: TargetFilter::Player,
            combat_only: true,
        },
        intervening_if: None,
        effect: catgeist_damage,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![],
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(damage_trig),
    )
}

fn catgeist_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
