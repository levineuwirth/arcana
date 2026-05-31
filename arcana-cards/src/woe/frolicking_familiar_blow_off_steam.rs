//! Frolicking Familiar // Blow Off Steam — `{2}{U}` Otter Wizard creature
//! 2/2 with Flying and "Whenever you cast an instant or sorcery spell, this
//! creature gets +1/+1 until end of turn." Adventure face "Blow Off Steam"
//! (`{R}` instant): "Blow Off Steam deals 1 damage to any target."
//!
//! # Rules references
//! * CR 715 — Adventurer. The card has a creature main face and an Adventure
//!   instant face; casting the Adventure exiles the card so the creature may be
//!   cast later from exile.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frolicking Familiar");
    let otter = reg.interner_mut().intern("Otter");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid creature cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // Adventure face "Blow Off Steam" — {R} instant, 1 damage to any target.
    let adv_name = reg.interner_mut().intern("Blow Off Steam");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid adventure cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Blow Off Steam deals 1 damage to any target.".into(),
        target_requirements: vec![TargetRequirement::any_target()],
        modal: None,
        effect: blow_off_steam_resolve,
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
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::INSTANT | TypeLine::SORCERY,
                    ))),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blow_off_steam_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 1,
    }]
}

fn pump_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
