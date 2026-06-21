//! Toby, Beastie Befriender — `{2}{W}` 1/1 Legendary Creature — Human Wizard.
//! "When Toby enters, create a 4/4 white Beast creature token with 'This token
//! can't attack or block alone.'" "As long as you control four or more creature
//! tokens, creature tokens you control have flying."

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
    let name = reg.interner_mut().intern("Toby, Beastie Befriender");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: static "as long as you control four or more creature tokens,
    // creature tokens you control have flying" — conditional board-wide
    // keyword grant is not expressible in this card class.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
            effect: make_beast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_beast(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token's quoted "This token can't attack or block alone" ability
    // is not expressible; the token is created with correct bones.
    let beast = reg.interner().lookup("Beast").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: beast,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
