//! Lich-Knights' Conquest — `{4}{B}` sorcery. "Sacrifice any number of
//! artifacts, enchantments, and/or tokens. Return that many creature
//! cards from your graveyard to the battlefield." The variable sacrifice
//! is modeled via a player-chosen any-number pick.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lich-Knights' Conquest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Sacrifice any number of artifacts, enchantments, and/or tokens. Return that many creature cards from your graveyard to the battlefield.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Sacrifice any number of artifacts, enchantments, and/or tokens" is
    // a variable player-chosen pick over your artifacts/enchantments.
    // GAP (count readback): "Return that many creature cards from your
    // graveyard to the battlefield" — there is no way to read back the
    // number sacrificed by ChooseAnyNumberFromZone and feed it into a
    // graveyard-return count, so the reanimation rider is dropped.
    // GAP (filter): the "and/or tokens" clause cannot be OR-ed with the
    // type filter; pure tokens that are neither artifacts nor enchantments
    // are not offered as sacrifice fodder.
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: entry.controller,
        zone: Zone::Battlefield,
        filter: ObjectFilter::permanent()
            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT))
            .controlled_by(ControllerConstraint::You),
        action: PickAction::Sacrifice,
    }]
}
