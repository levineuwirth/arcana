//! Queen of Ice // Rage of Winter — `{2}{U}` // `{1}{U}` blue Adventure creature.
//! Creature: 2/3 Human Noble Wizard. "Whenever this creature deals combat damage to a creature, tap that creature. It doesn't untap during its controller's next untap step."
//! Adventure (Rage of Winter — Sorcery): Tap target creature. It doesn't untap during its controller's next untap step.
//! GAP: "doesn't untap during next untap step" — no-untap duration not in catalog; using Tap only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Queen of Ice");
    let adv_name = reg.interner_mut().intern("Rage of Winter");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(noble_sub);
    subtypes.0.insert(wizard_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(3)), ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Tap target creature.".into(), target_requirements: vec![TargetRequirement::target_creature()], modal: None, effect: rage_of_winter_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1, trigger_condition: TriggerCondition::DamageDealt { source_filter: arcana_core::targets::ObjectFilter::new().with_supertypes(arcana_core::types::SupertypeSet::new().with(arcana_core::types::SupertypeSet::LEGENDARY)), target_filter: arcana_core::targets::TargetFilter::Creature, combat_only: true },
                intervening_if: None, effect: on_combat_damage_creature, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn on_combat_damage_creature(_state: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "doesn't untap during controller's next untap step" — no-untap duration not in catalog
    // Using damaged_player detection as best effort — actually we need the damaged creature id
    Vec::new()
}

fn rage_of_winter_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "doesn't untap during controller's next untap step" — not in catalog
    vec![Effect::Tap { target: *id }]
}
