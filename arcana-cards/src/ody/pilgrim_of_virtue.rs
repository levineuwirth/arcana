//! Pilgrim of Virtue — `{2}{W}` 1/3 Human Cleric.
//! "Protection from black"
//! "{W}, Sacrifice this creature: The next time a black source of your choice
//!  would deal damage this turn, prevent that damage."
//!
//! GAP: Protection (from black) is not an available KeywordAbility variant; the
//! keyword line is omitted.
//! Fidelity GAP: "a black source of your choice ... the next time" is a single
//! chosen one-shot prevention; the engine only offers source-filtered board-wide
//! prevention. Implemented as "prevent all damage from black permanents to any
//! target this turn" via PreventDamageFrom — broader than the printed effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pilgrim of Virtue");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Protection from black not an available keyword.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}, Sacrifice this creature: The next time a black source of your \
                   choice would deal damage this turn, prevent that damage."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: prevent_black_damage,
        }),
    )
}

fn prevent_black_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::permanent().with_colors(ColorSet::black()),
        target_filter: TargetFilter::AnyTarget,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
