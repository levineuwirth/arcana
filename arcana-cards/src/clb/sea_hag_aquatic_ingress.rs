//! Sea Hag // Aquatic Ingress — `{4}{U}` // `{2}{U}` blue Adventure creature.
//! Creature: 3/5 Hag. When this creature enters, creatures opponents control get -4/-0 until end of turn.
//! Adventure (Aquatic Ingress — Instant): Up to two target creatures each get +1/+0 until end of turn and can't be blocked this turn.
//! GAP: "can't be blocked this turn" — unblockable effect not in catalog.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sea Hag");
    let adv_name = reg.interner_mut().intern("Aquatic Ingress");
    let hag_sub = reg.interner_mut().intern("Hag");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hag_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(5)), ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::INSTANT.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef {
        text: "Up to two target creatures each get +1/+0 until end of turn.".into(),
        target_requirements: vec![TargetRequirement { filter: TargetFilter::Creature, count: TargetCount::UpTo(2), controller: None }],
        modal: None,
        effect: aquatic_ingress_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::SelfEntersBattlefield, intervening_if: None, effect: etb_debuff, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_adventure(adventure),
    )
}

fn etb_debuff(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent), trig.controller);
    ids.into_iter().map(|id| Effect::Pump { target: id, power: -4, toughness: 0, duration: Duration::EndOfTurn, keywords: vec![] }).collect()
}

fn aquatic_ingress_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "can't be blocked this turn" — unblockable not in catalog
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::Pump { target: *id, power: 1, toughness: 0, duration: Duration::EndOfTurn, keywords: vec![] })
        } else { None }
    }).collect()
}
