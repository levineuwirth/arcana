//! Come Back Wrong — `{2}{B}` sorcery. "Destroy target creature. If a
//! creature card is put into a graveyard this way, return it to the
//! battlefield under your control. Sacrifice it at the beginning of
//! your next end step." The graveyard-bounce-back conditional on
//! 'put into a graveyard this way' isn't expressible as a single
//! catalog primitive — best effort: destroy now, then GAP the
//! conditional reanimate-with-EOT-sac rider.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Come Back Wrong");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. If a creature card is put into a graveyard this way, return it to the battlefield under your control. Sacrifice it at the beginning of your next end step.".into(),
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
    // GAP: 'if a creature card is put into a graveyard this way, return it
    // to the battlefield under your control. Sacrifice it at the beginning
    // of your next end step.' — the catalog has no conditional 'on
    // graveyard-arrival from this destroy' hook to chain a steal+EOT-sac.
    vec![Effect::DestroyPermanent { target: *id }]
}
