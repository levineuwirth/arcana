//! Soul Servitude — `{2}{B}` instant. "Target player sacrifices a
//! nontoken creature. When they do, you may discard a card. If you
//! do, conjure a duplicate of the sacrificed creature into your hand.
//! It perpetually gains '...'" GAP: Conjure / perpetual gains and
//! triggered-on-sac follow-up aren't in the catalog. Express only the
//! initial sac.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Servitude");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player sacrifices a nontoken creature. When they do, you may discard a card. If you do, conjure a duplicate of the sacrificed creature into your hand. It perpetually gains \"You may spend mana as though it were mana of any color to cast this spell.\"".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    // GAP: conjure-duplicate + perpetual-gains; only initial sac expressed.
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    vec![Effect::Sacrifice {
        player: *p,
        filter: ObjectFilter::creature().nontoken(),
        count: 1,
    }]
}
