//! Rashmi and Ragavan — `{1}{G}{U}{R}` 2/4 legendary green-blue-red Elf Monkey.
//! "Whenever you cast your first spell during each of your turns, exile the top
//! card of target opponent's library and create a Treasure token. Then you may
//! cast the exiled card without paying its mana cost if it's a spell with mana
//! value less than the number of artifacts you control."
//!
//! GAP: "first spell during each of your turns" tracking; exile top of opponent's
//! library; conditional free cast based on artifact count — none fully expressible.
//! Creating Treasure token only as best effort.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rashmi and Ragavan");
    let elf = reg.interner_mut().intern("Elf");
    let monkey = reg.interner_mut().intern("Monkey");
    let _treasure = reg.interner_mut().intern("Treasure");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(monkey);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: on_spell_cast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_spell_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile top of opponent's library + conditional free cast not expressible.
    let treasure = reg.interner().lookup("Treasure").expect("Treasure interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treasure);
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
