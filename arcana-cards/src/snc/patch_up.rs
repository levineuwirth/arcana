//! Patch Up — `{2}{W}` sorcery. "Return up to three target creature
//! cards with total mana value 3 or less from your graveyard to the
//! battlefield." Total-mana-value-bound across multiple targets isn't
//! in catalog; we emit up-to-three reanimates and the engine prompts
//! honestly without the total-mv constraint.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patch Up");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to three target creature cards with total mana value 3 or less from your graveyard to the battlefield.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_cmc(3),
                },
                count: TargetCount::UpTo(3),
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
    // GAP: TOTAL mana value <= 3 across chosen set not expressible in TargetCount/ObjectFilter.
    let mut effects = Vec::new();
    for choice in &entry.targets.targets {
        if let TargetChoice::Object(id) = choice {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    effects
}
