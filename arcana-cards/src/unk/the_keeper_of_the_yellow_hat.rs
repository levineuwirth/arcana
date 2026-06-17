//! The Keeper of the Yellow Hat — `{W}{U}` 1/1 Legendary Human Wizard.
//!
//! * "You can't cast Keeper of the Yellow Hat during your first seven turns of the
//!   game." GAP: turn-gated casting restriction is not expressible.
//! * When it enters, create a legendary artifact Equipment token named Yellow Hat with
//!   equip {2} and "Equipped creature gets +4/+4 and gains lifelink." We mint a bare
//!   artifact Equipment token; GAP the equip activated ability, the legendary supertype
//!   (TokenDefinition has no supertypes field), and the equipped-creature static.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Keeper of the Yellow Hat");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _hat = reg.interner_mut().intern("Yellow Hat");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: keeper_make_hat,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn keeper_make_hat(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let hat = reg.interner().lookup("Yellow Hat").unwrap_or_default();
    let equipment = reg.interner().lookup("Equipment").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: hat,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
