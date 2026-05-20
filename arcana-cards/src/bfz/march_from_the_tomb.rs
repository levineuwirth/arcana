//! March from the Tomb — `{3}{W}{B}` sorcery. "Return any number of
//! target Ally creature cards with total mana value 8 or less from your
//! graveyard to the battlefield."

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
    let name = reg.interner_mut().intern("March from the Tomb");
    let _ally = reg.interner_mut().intern("Ally");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return any number of target Ally creature cards with total mana value 8 or less from your graveyard to the battlefield.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    // Ally subtype + the "total MV 8 or less" aggregate
                    // cap have no expressible form here; broad creature
                    // filter, with the constraint noted as a GAP.
                    filter: ObjectFilter::creature().with_max_cmc(8),
                },
                count: TargetCount::Any,
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: cannot constrain to the "Ally" subtype in a graveyard
    // TargetFilter, nor enforce the aggregate "total mana value 8 or
    // less" across the chosen set; returning each chosen card.
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
            }
            _ => None,
        })
        .collect()
}
