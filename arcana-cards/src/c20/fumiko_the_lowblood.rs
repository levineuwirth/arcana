//! Fumiko the Lowblood — `{2}{R}{R}` 3/2 Legendary Human Samurai.
//! "Fumiko has bushido X, where X is the number of attacking creatures.
//!  Creatures your opponents control attack each combat if able."
//!
//! Bushido is a parametrized keyword taking a FIXED u8 — "bushido X where X is
//! the number of attacking creatures" is a dynamic value the keyword can't
//! carry, so it is GAP'd rather than misrepresented as a fixed Bushido(N).
//! The "opponents' creatures attack each combat if able" line is wired as an
//! ETB-installed `filtered_must_attack` over opponents' creatures (CR 508.1a).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fumiko the Lowblood");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "bushido X, where X is the number of attacking creatures" — Bushido(N) takes a fixed u8, not a dynamic X.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_opponents_must_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: "Creatures your opponents control attack each combat if able."
/// (CR 508.1a) — board-wide must-attack over opponents' creatures.
fn install_opponents_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_must_attack(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
