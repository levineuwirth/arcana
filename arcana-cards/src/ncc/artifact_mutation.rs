//! Artifact Mutation — `{R}{G}` instant. "Destroy target artifact. It
//! can't be regenerated. Create X 1/1 green Saproling creature tokens,
//! where X is that artifact's mana value."
//!
//! The destroy is expressible. The token rider's count X is the
//! destroyed artifact's mana value, which is read from the target
//! object — there is no `script::` helper to fetch a target permanent's
//! mana value, so the token clause cannot be computed and is GAP-ed
//! rather than emitted with a wrong fixed count.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Artifact Mutation");
    let _saproling = reg.interner_mut().intern("Saproling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target artifact. It can't be regenerated. \
                       Create X 1/1 green Saproling creature tokens, where \
                       X is that artifact's mana value."
                    .into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
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
    // GAP: "can't be regenerated" rider on a destroy is not expressible.
    // GAP: token count X = destroyed artifact's mana value; no script::
    //      helper reads a target permanent's mana value, so the Saproling
    //      token clause cannot be computed (a fixed count would be wrong).
    vec![Effect::DestroyPermanent { target: *id }]
}
