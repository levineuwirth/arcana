//! Chea, Friend to Maybe Too Many — `{1}{W}{B}{G}` 2/4 Legendary Human Wizard.
//! "Familiar spells you cast have flash." (GAP — no cost/permission-granting
//! static for a card-subset.)
//! "{T}: Add X {G} and each opponent loses X life, where X is the number of
//! familiars you control." (The "any creature with 'Familiar' in its name"
//! clause is unexpressible; X counts the listed familiar subtypes only.)

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

const FAMILIAR_SUBTYPES: [&str; 10] = [
    "Bat", "Bird", "Cat", "Dragon", "Faerie", "Fox", "Frog", "Imp", "Lizard",
    "Spider",
];

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chea, Friend to Maybe Too Many");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: static "Familiar spells you cast have flash".
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add X {G} and each opponent loses X life, where X is the number of familiars you control.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_mana_and_drain,
            }),
    )
}

fn add_mana_and_drain(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let syms: Vec<_> = FAMILIAR_SUBTYPES
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(syms);
    let n = script::count_matching(state, &filter, ctx.controller);
    let mut effects = vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); n as usize],
    }];
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::LoseLife { player: opp, amount: n });
    }
    effects
}
