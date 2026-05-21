//! Lithobraking — `{2}{R}` instant. Create a Lander token. Then you may
//! sacrifice an artifact. When you do, Lithobraking deals 2 damage to
//! each creature. (Lander token activated tutor not modeled;
//! conditional follow-up emitted unconditionally as best-effort.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lithobraking");
    let _lander = reg.interner_mut().intern("Lander");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a Lander token. Then you may sacrifice an artifact. When you do, Lithobraking deals 2 damage to each creature.".into(),
            target_requirements: vec![],
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
    // GAP: "may sacrifice an artifact. When you do, …" — conditional reflexive trigger
    // not modeled; we emit the sacrifice + damage best-effort.
    let lander = reg
        .interner()
        .lookup("Lander")
        .expect("Lander interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lander);
    let token = TokenDefinition {
        name: lander,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects: Vec<Effect> = vec![
        Effect::CreateToken {
            controller: entry.controller,
            token,
        },
        Effect::Sacrifice {
            player: entry.controller,
            filter: ObjectFilter::permanent()
                .controlled_by(ControllerConstraint::You)
                .with_types(TypeLine::ARTIFACT.into()),
            count: 1,
        },
    ];
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    for id in creatures {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 2,
        });
    }
    effects
}
