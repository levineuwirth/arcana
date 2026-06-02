//! Might of Alara — `{G}` instant. "Domain — Target creature gets
//! +1/+1 until end of turn for each basic land type among lands you
//! control." The pump amount scales with the number of distinct basic
//! land types (Plains, Island, Swamp, Mountain, Forest) among lands
//! you control (CR 700.6 Domain).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Might of Alara");
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Target creature gets +1/+1 until end of turn for each basic land type among lands you control.".into(),
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
    // Domain: count distinct basic land types among lands you control.
    let mut domain: i32 = 0;
    for kind in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let filter = script::subtype_filter(reg, kind);
        if script::count_matching(state, &filter, entry.controller) > 0 {
            domain += 1;
        }
    }
    vec![Effect::Pump {
        target: *id,
        power: domain,
        toughness: domain,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
