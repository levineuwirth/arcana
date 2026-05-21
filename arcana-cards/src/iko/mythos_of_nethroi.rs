//! Mythos of Nethroi — `{2}{B}` instant. "Destroy target nonland
//! permanent if it's a creature or if {G}{W} was spent to cast this
//! spell." We can target a nonland permanent; the conditional destroy
//! (only fires if creature OR colored-mana-spent) — colored-mana
//! tracking is not in catalog. We emit destroy unconditionally for the
//! creature branch and rely on the target filter to restrict to
//! creatures; GAP the GW-spent branch.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mythos of Nethroi");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target nonland permanent if it's a creature or if {G}{W} was spent to cast this spell.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "{G}{W} was spent to cast this" branch (nonland permanent destroy) not in catalog.
    vec![Effect::DestroyPermanent { target: *id }]
}
