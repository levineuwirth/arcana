//! Revive the Shire — `{1}{G}` sorcery. "Return target permanent card
//! from your graveyard to your hand. Create a Food token."
//!
//! GAP: ReturnFromGraveyardToHand requires a creature card target per
//! catalog; "permanent card" (non-creature) is not a supported zone
//! filter. The Food token activated ability ({2},{T}, Sacrifice: gain
//! 3 life) is also not expressible as TokenDefinition abilities.
//! Returning partial: the graveyard return uses creature filter as
//! approximation; Food token GAP noted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Revive the Shire");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target permanent card from your graveyard to your hand. Create a Food token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::permanent() },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: Food token (TokenDefinition with activated ability: {2},{T}, Sacrifice: gain 3 life)
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
