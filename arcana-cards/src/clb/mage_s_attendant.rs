//! Mage's Attendant — `{2}{W}` 3/2 white creature. "When this creature
//! enters, create a 1/1 blue Wizard creature token with '{1}, Sacrifice
//! this token: Counter target noncreature spell unless its controller
//! pays {1}.'"
//!
//! GAP: effect — the token's activated ability (counter a spell unless
//! mana is paid) is not expressible in the TokenDefinition abilities field
//! with the current API. The token is created without that ability.

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
    let name = reg.interner_mut().intern("Mage's Attendant");
    let cat = reg.interner_mut().intern("Cat");
    let rogue = reg.interner_mut().intern("Rogue");
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_wizard_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_wizard_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wizard = reg.interner().lookup("Wizard")
        .expect("Wizard interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(wizard);
    let token = TokenDefinition {
        name: wizard,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
        // GAP: effect — token's "{1}, Sacrifice: counter noncreature spell unless {1}" ability not expressible
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
