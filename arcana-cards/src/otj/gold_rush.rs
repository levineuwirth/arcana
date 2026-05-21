//! Gold Rush — `{1}{G}` instant. "Create a Treasure token. Until end
//! of turn, up to one target creature gets +2/+2 for each Treasure
//! you control." Treasure token (artifact, "{T}, Sacrifice: add one
//! mana of any color") activated ability not in TokenDefinition —
//! emit a plain Treasure artifact token. Per-Treasure +2/+2 needs us
//! to count Treasures (subtype) and multiply the pump by N. We count
//! BEFORE creating this new one so the +2/+2 uses the pre-resolution
//! Treasure count, matching the printed timing.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gold Rush");
    let _treasure = reg.interner_mut().intern("Treasure");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    // GAP: Treasure token's '{T}, Sacrifice this token: Add one mana of any color'.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a Treasure token. Until end of turn, up to one target creature gets +2/+2 for each Treasure you control.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
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
    let treasure = reg.interner().lookup("Treasure").expect("Treasure interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Treasure").controlled_by(ControllerConstraint::You),
        entry.controller,
    ) as i32
        + 1; // include the one we're about to create
    let mut effects = vec![Effect::CreateToken { controller: entry.controller, token }];
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::Pump {
            target: *id,
            power: 2 * n,
            toughness: 2 * n,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}
