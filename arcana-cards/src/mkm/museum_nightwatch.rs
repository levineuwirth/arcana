//! Museum Nightwatch — `{3}{W}` 3/2 Creature — Centaur Soldier.
//!
//! * "When this creature dies, create a 2/2 white and blue Detective creature
//!   token." — a `SelfDies` trigger minting the token.
//! * "Disguise {1}{W}" — GAP: Disguise (face-down cast as a 2/2 with ward {2},
//!   turn face up) is not a usable keyword variant and the morph-family
//!   mechanic is not expressible.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Museum Nightwatch");
    let centaur = reg.interner_mut().intern("Centaur");
    let soldier = reg.interner_mut().intern("Soldier");
    let _detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: make_detective_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: "Disguise {1}{W}" — face-down-cast morph-family keyword, not usable.
}

fn make_detective_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let detective = reg
        .interner()
        .lookup("Detective")
        .expect("Detective interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(detective);
    let token = TokenDefinition {
        name: detective,
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
