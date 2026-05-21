//! Geosurge — `{R}{R}{R}{R}` sorcery. "Add {R}{R}{R}{R}{R}{R}{R}. Spend this
//! mana only to cast artifact or creature spells."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Geosurge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Add {R}{R}{R}{R}{R}{R}{R}. Spend this mana only to cast artifact or creature spells.".into(),
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
    // Note: the "spend only on artifact/creature spells" mana-use restriction is not
    // representable; the mana is added unrestricted.
    vec![Effect::AddMana {
        player: entry.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, entry.source); 7],
    }]
}
