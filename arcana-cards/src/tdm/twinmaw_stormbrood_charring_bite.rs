//! Twinmaw Stormbrood // Charring Bite — `{5}{W}` Creature — Dragon 5/4 with
//! flying; "When this creature enters, you gain 5 life." Adventure face
//! "Charring Bite" — `{1}{R}` Sorcery — Omen, "Charring Bite deals 5 damage to
//! target creature without flying."
//!
//! CR 715 Adventurer. The creature half is the main face; Charring Bite is the
//! Adventure (Omen) face, cast from hand then exiled, castable later as the
//! Dragon.
//!
//! GAP: the Adventure face's "target creature WITHOUT flying" restriction has
//! no ObjectFilter keyword-exclusion primitive — we target any creature and
//! lose the no-flying clause.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetChoice;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Twinmaw Stormbrood");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);

    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid creature cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // Adventure face "Charring Bite" — {1}{R} Sorcery — Omen.
    let adv_name = reg.interner_mut().intern("Charring Bite");
    let omen_sub = reg.interner_mut().intern("Omen");
    let mut adv_subtypes = SubtypeSet::default();
    adv_subtypes.0.insert(omen_sub);
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid adventure cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        subtypes: adv_subtypes,
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Charring Bite deals 5 damage to target creature without flying.".into(),
        // GAP: "without flying" restriction not expressible; target any creature.
        target_requirements: vec![TargetRequirement::target_creature()],
        modal: None,
        effect: charring_bite_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure)
            // When this creature enters, you gain 5 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn etb_gain_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 5 }]
}

fn charring_bite_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 5,
    }]
}
