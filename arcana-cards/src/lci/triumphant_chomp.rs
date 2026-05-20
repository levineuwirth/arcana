//! Triumphant Chomp — `{R}` sorcery. "Triumphant Chomp deals damage equal
//! to 2 or the greatest power among Dinosaurs you control, whichever is
//! greater, to target creature."
//! Uses script::subtype_filter + script::ids_matching + script::power_of to
//! find the maximum power among your Dinosaurs, then takes max(2, that).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Triumphant Chomp");
    let _dinosaur = reg.interner_mut().intern("Dinosaur");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Triumphant Chomp deals damage equal to 2 or the greatest power among Dinosaurs you control, whichever is greater, to target creature.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let dino_filter = script::subtype_filter(reg, "Dinosaur");
    let dino_ids = script::ids_matching(state, &dino_filter, entry.controller);
    let max_power = dino_ids.iter()
        .map(|&did| script::power_of(state, did).max(0) as u32)
        .max()
        .unwrap_or(0);
    let amount = max_power.max(2);
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
